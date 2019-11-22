package ttaomae.timecalc;

import javafx.fxml.FXML;
import javafx.scene.input.KeyCode;
import javafx.scene.layout.VBox;
import ttaomae.timecalc.control.Display;
import ttaomae.timecalc.control.Keypad;
import ttaomae.timecalc.util.ExpressionEvalutor;
import ttaomae.timecalc.util.ExpressionFormatter;
import ttaomae.timecalc.util.TimeCalcCoreExpressionEvaluator;
import ttaomae.timecalc.core.TimeCalcLoader;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class TimeCalculatorController
{
    @FXML private VBox root;
    @FXML private Display display;
    @FXML private Keypad keypad;

    private final ExpressionFormatter formatter;
    private final ExpressionEvalutor evalutor;

    public TimeCalculatorController()
    {
        formatter = new ExpressionFormatter();
        evalutor = new TimeCalcCoreExpressionEvaluator(loadTimeCalcCore());
    }

    private static Path loadTimeCalcCore()
    {
        try (var exe = TimeCalcLoader.class.getResourceAsStream("/time-calc.exe")) {
            var tempDir = Files.createTempDirectory("time-calc-");
            tempDir.toFile().deleteOnExit();

            var timeCalcPath = tempDir.resolve("tc.exe");
            Files.copy(exe, timeCalcPath);
            return timeCalcPath;
        }
        catch (IOException e) {
            throw new IllegalStateException("Could not load time-calc/core executable.");
        }
    }

    @FXML private void initialize()
    {
        for (Keypad.Key key : Keypad.Key.values()) {
            keypad.setOnAction(key, event -> updateDisplay(key));
        }
        root.setOnKeyTyped(keyEvent -> updateDisplay(keyEvent.getCharacter()));
        root.setOnKeyPressed(keyEvent -> updateDisplay(keyEvent.getCode()));
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
        // Move answer into input. Start by assuming it is a number.
        formatter.inputCharacter(ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        var isTime = false;
        for (char ch : display.getResultText().toCharArray()) {
            // It is a time if there is a colon or s.
            if (ch == ':' || ch == 's') isTime = true;
            formatter.inputCharacter(ch);
        }
        // Toggle type back to time.
        if (isTime) formatter.inputCharacter(ExpressionFormatter.TOGGLE_TYPE_CHARACTER);

        display.setInputText(formatter.toString());
    }

    private void delete()
    {
        var expression = formatter.deleteCharacter();
        display.setInputText(expression);
        var result = evalutor.evaluate(expression);
        if (result.isSuccess()) {
            display.setResultText(result.getValue().get());
        }
    }

    private void evaluate(Keypad.Key key)
    {
        String expression = formatter.inputCharacter(key.charValue());
        display.setInputText(expression);
        var result = evalutor.evaluate(expression);
        if (result.isSuccess()) {
            display.setResultText(result.getValue().get());
        }
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
            default:
                evaluate(key);
                break;
        }
    }

    private void updateDisplay(String character)
    {
        Keypad.Key.fromCharacter(character).ifPresent(this::updateDisplay);
    }

    private void updateDisplay(KeyCode keyCode)
    {
        Keypad.Key.fromKeyCode(keyCode).ifPresent(this::updateDisplay);
    }
}
