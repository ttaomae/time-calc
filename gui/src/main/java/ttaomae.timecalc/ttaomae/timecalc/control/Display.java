package ttaomae.timecalc.control;

import javafx.beans.property.SimpleStringProperty;
import javafx.beans.property.StringProperty;
import javafx.scene.control.Control;
import javafx.scene.control.Label;
import javafx.scene.control.Skin;
import javafx.scene.control.SkinBase;
import javafx.scene.layout.VBox;

public class Display extends Control
{
    public Display()
    {
        getStyleClass().setAll("display");
    }

    // Input text property.
    private final StringProperty inputTextProperty = new SimpleStringProperty("");
    public StringProperty inputTextProperty() { return inputTextProperty; }
    public String getInputText() { return inputTextProperty().get(); }
    public void setInputText(String text) { inputTextProperty().set(text); }

    // Result text property.
    private final StringProperty resultTextProperty = new SimpleStringProperty("");
    public StringProperty resultTextProperty() { return resultTextProperty; }
    public String getResultText() { return resultTextProperty().get(); }
    public void setResultText(String text) { resultTextProperty().setValue(text); }

    @Override
    protected Skin<?> createDefaultSkin()
    {
        return new DisplaySkin(this);
    }

    private static class DisplaySkin extends SkinBase<Display>
    {
        private DisplaySkin(Display display) {
            super(display);

            var inputLabel = new Label();
            inputLabel.getStyleClass().add("input");
            inputLabel.textProperty().bind(display.inputTextProperty());

            var resultLabel = new Label();
            resultLabel.getStyleClass().add("result");
            resultLabel.textProperty().bind(display.resultTextProperty());

            getChildren().add(new VBox(inputLabel, resultLabel));
        }
    }
}
