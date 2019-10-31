package ttaomae.timecalc;

import javafx.fxml.FXML;
import javafx.scene.input.KeyCode;
import javafx.scene.layout.VBox;
import ttaomae.timecalc.control.Display;
import ttaomae.timecalc.control.Keypad;
import ttaomae.timecalc.util.ExpressionFormatter;

public class TimeCalculatorController
{
    @FXML private VBox root;
    @FXML private Display display;
    @FXML private Keypad keypad;

    private final ExpressionFormatter formatter;

    public TimeCalculatorController()
    {
        formatter = new ExpressionFormatter();
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
        display.setText("");
        formatter.clear();
    }

    private void updateDisplay(Keypad.Key key)
    {
        switch (key) {
            case CLEAR:
            case EQUALS:
                clear();
                break;
            default:
                display.setText(formatter.inputCharacter(key.charValue()));
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
