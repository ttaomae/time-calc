package ttaomae.timecalc.control;

import javafx.beans.property.SimpleStringProperty;
import javafx.beans.property.StringProperty;
import javafx.scene.control.Control;
import javafx.scene.control.Label;
import javafx.scene.control.Skin;
import javafx.scene.control.SkinBase;

public class Display extends Control
{
    public Display()
    {
        getStyleClass().setAll("display");
    }

    private final StringProperty textProperty = new SimpleStringProperty("");
    public StringProperty textProperty() { return textProperty; }
    public String getText() { return textProperty().get(); }
    public void setText(String text) { textProperty().set(text); }

    @Override
    protected Skin<?> createDefaultSkin()
    {
        return new DisplaySkin(this);
    }

    private class DisplaySkin extends SkinBase<Display>
    {
        private DisplaySkin(Display display) {
            super(display);

            var label = new Label();
            label.textProperty().bind(display.textProperty());

            getChildren().add(label);
        }
    }
}
