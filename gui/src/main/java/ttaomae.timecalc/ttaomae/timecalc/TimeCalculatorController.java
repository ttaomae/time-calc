package ttaomae.timecalc;

import javafx.fxml.FXML;
import ttaomae.timecalc.control.Display;
import ttaomae.timecalc.control.Keypad;

public class TimeCalculatorController
{
    @FXML private Display display;
    @FXML private Keypad keypad;

    @FXML private void initialize()
    {
        for (Keypad.Key key : Keypad.Key.values()) {
            keypad.setOnAction(key, event -> display.setText(display.getText() + key));
        }

        keypad.setOnAction(Keypad.Key.EQUALS, event -> display.setText(""));
        keypad.setOnAction(Keypad.Key.CLEAR, event -> display.setText(""));
    }
}
