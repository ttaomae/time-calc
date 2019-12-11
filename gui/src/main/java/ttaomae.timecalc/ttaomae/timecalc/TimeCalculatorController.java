package ttaomae.timecalc;

import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.scene.input.KeyCode;
import javafx.scene.layout.VBox;
import ttaomae.timecalc.control.Display;
import ttaomae.timecalc.control.Keypad;
import ttaomae.timecalc.util.ExpressionEvalutor;
import ttaomae.timecalc.util.ExpressionFormatter;
import ttaomae.timecalc.core.TimeCalcLoader;
import ttaomae.timecalc.util.InteractiveModeTimeCalcCore;

import java.io.IOException;
import java.nio.file.FileVisitResult;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.SimpleFileVisitor;
import java.nio.file.attribute.BasicFileAttributes;
import java.nio.file.attribute.PosixFilePermissions;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.logging.Logger;

public class TimeCalculatorController
{
    private static final Logger LOGGER =
            Logger.getLogger(TimeCalculatorController.class.getCanonicalName());

    @FXML private VBox root;
    @FXML private Display display;
    @FXML private Keypad keypad;

    private final ExpressionFormatter formatter;
    private final ExpressionEvalutor evaluator;
    private final ExecutorService evaluatorThreadPool;

    public TimeCalculatorController() throws IOException
    {
        formatter = new ExpressionFormatter();
        var temporaryDirectory = Files.createTempDirectory("time-calc-");
        var timeCalcPath = loadTimeCalcCore(temporaryDirectory);
        evaluator = new InteractiveModeTimeCalcCore(timeCalcPath);
        evaluatorThreadPool = createEvaluatorThreadPool();

        Runtime.getRuntime().addShutdownHook(newCleanupThread(
                evaluator, evaluatorThreadPool, temporaryDirectory));
    }

    /**
     * Loads the time-calc/core executable into the specified directory.
     */
    private static Path loadTimeCalcCore(Path outputDirectory)
    {
        try (var executableStream = TimeCalcLoader.getExecutableAsStream()) {
            var timeCalcPath = outputDirectory.resolve(TimeCalcLoader.getExecutableName());
            Files.copy(executableStream, timeCalcPath);

            try {
                // JAR files do not preserve permissions of the files they contain, so we need to
                // ensure that the copied file is executable
                Files.setPosixFilePermissions(timeCalcPath,
                        PosixFilePermissions.fromString("rwxr-xr-x"));
            }
            catch (UnsupportedOperationException ignored) {
                // If this is a non-POSIX file system, assume that the file is already executable
                // and ignore the exception.
            }

            return timeCalcPath;
        }
        catch (IOException e) {
            throw new IllegalStateException("Could not load time-calc/core executable.", e);
        }
    }

    @FXML private void initialize()
    {
        for (Keypad.Key key : Keypad.Key.values()) {
            keypad.setOnAction(key, event -> updateDisplay(key));
        }
        root.setOnKeyTyped(keyEvent -> onKeyTyped(keyEvent.getCharacter()));
        root.setOnKeyPressed(keyEvent -> onKeyPressed(keyEvent.getCode()));
    }

    private void clear()
    {
        display.setInputText("");
        formatter.clear();
        display.setResultText("");
    }

    private void evaluate()
    {
        formatter.clear();

        if (display.getResultText().isEmpty()) return;

        var isNumber = true;
        char[] chars = display.getResultText().toCharArray();
        var isNegative = chars[0] == '-';
        // Input character from output into input.
        // All characters are input, but some may be ignored.
        for (char ch : chars) {
            // It is a time if there is a colon or s.
            if (ch == ':' || ch == 's') isNumber = false;
            formatter.inputCharacter(ch);
        }

        // Convert to appropriate type and sign.
        if (isNumber) formatter.inputCharacter(ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        if (isNegative) formatter.inputCharacter(ExpressionFormatter.TOGGLE_SIGN_CHARACTER);

        display.setInputText(formatter.toString());
    }

    private void delete()
    {
        var expression = formatter.deleteCharacter();
        updateDisplay(expression);
    }

    private void inputCharacter(Keypad.Key key)
    {
        var expression = formatter.inputCharacter(key.charValue());
        updateDisplay(expression);
    }

    private void updateDisplay(String expression)
    {
        display.setInputText(expression);
        evaluatorThreadPool.submit(() ->
            // If value is present, then evaluation succeeded. Only update display on success.
            evaluator.evaluate(expression).getValue().ifPresent(this::setResult));
    }

    /**
     * Sets the display's result text to the specified value. Runs on the JavaFX Application Thread.
     */
    private void setResult(String result)
    {
        Platform.runLater(() -> display.setResultText(result));
    }

    private void updateDisplay(Keypad.Key key)
    {
        switch (key) {
            case CLEAR:
                clear();
                break;
            case EQUALS:
                evaluate();
                break;
            case DELETE:
                delete();
                break;
            default:
                inputCharacter(key);
                break;
        }
    }

    private void onKeyTyped(String character)
    {
        Keypad.Key.fromCharacter(character).ifPresent(this::updateDisplay);
    }

    private void onKeyPressed(KeyCode keyCode)
    {
        Keypad.Key.fromKeyCode(keyCode).ifPresent(this::updateDisplay);
    }

    /**
     * Creates a thread pool used to run tasks to evaluate expressions.
     */
    @SuppressWarnings("PMD.DoNotUseThreads")
    private static ExecutorService createEvaluatorThreadPool()
    {
        return Executors.newSingleThreadExecutor(runnable -> {
            Thread thread = new Thread(runnable);
            thread.setDaemon(true);
            thread.setName("time-calc-evaluate");
            return thread;
        });
    }

    @SuppressWarnings("PMD.DoNotUseThreads")
    private static Thread newCleanupThread(ExpressionEvalutor evaluator,
            ExecutorService evaluatorThreadPool, Path temporaryDirectory)
    {
        return new Thread(() -> {
            LOGGER.warning("Cleaning up.");
            if (evaluator instanceof InteractiveModeTimeCalcCore) {
                ((InteractiveModeTimeCalcCore) evaluator).shutDown();
            }

            evaluatorThreadPool.shutdown();
            try {
                evaluatorThreadPool.awaitTermination(1, TimeUnit.SECONDS);
            }
            catch (InterruptedException ignored) {
                LOGGER.warning("Interrupted while waiting for evaluator thread to terminate.");
            }

            try {
                recursiveDelete(temporaryDirectory);
            }
            catch (IOException e) {
                LOGGER.warning(() -> "Could not delete temporary directory: " + temporaryDirectory);
            }
        });
    }

    /**
     * Recursively deletes a directory.
     */
    private static void recursiveDelete(Path directory) throws IOException
    {
        Files.walkFileTree(directory, new SimpleFileVisitor<>() {
            @Override
            public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) throws IOException {
                Files.delete(file);
                return FileVisitResult.CONTINUE;
            }

            @Override
            public FileVisitResult postVisitDirectory(Path dir, IOException e) throws IOException {
                Files.delete(dir);
                return FileVisitResult.CONTINUE;
            }
        });
    }
}
