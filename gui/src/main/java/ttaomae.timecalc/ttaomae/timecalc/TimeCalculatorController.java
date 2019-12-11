package ttaomae.timecalc;

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
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.PosixFilePermissions;

public class TimeCalculatorController
{
    @FXML private VBox root;
    @FXML private Display display;
    @FXML private Keypad keypad;

    private final ExpressionFormatter formatter;
    private final ExpressionEvalutor evaluator;

    public TimeCalculatorController()
    {
        formatter = new ExpressionFormatter();
        evaluator = new InteractiveModeTimeCalcCore(loadTimeCalcCore());
    }

    private static Path loadTimeCalcCore()
    {
        try (var executableStream = TimeCalcLoader.getExecutableAsStream()) {
            var tempDir = Files.createTempDirectory("time-calc-");
            tempDir.toFile().deleteOnExit();

            var timeCalcPath = tempDir.resolve(TimeCalcLoader.getExecutableName());
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
        var result = evaluator.evaluate(expression);
        // If value is present, then evaluation succeed. Only update on success.
        result.getValue().ifPresent(value -> display.setResultText(value));
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
}
