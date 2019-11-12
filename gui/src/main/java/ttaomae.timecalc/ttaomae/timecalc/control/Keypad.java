package ttaomae.timecalc.control;

import javafx.beans.property.ObjectProperty;
import javafx.beans.property.SimpleObjectProperty;
import javafx.event.ActionEvent;
import javafx.event.EventHandler;
import javafx.scene.control.Button;
import javafx.scene.control.Control;
import javafx.scene.control.Skin;
import javafx.scene.control.SkinBase;
import javafx.scene.input.KeyCode;
import javafx.scene.layout.GridPane;
import javafx.scene.layout.Priority;

import java.util.EnumMap;
import java.util.Optional;

public class Keypad extends Control
{
    public enum Key
    {
        ZERO("0"),
        ONE("1"),
        TWO("2"),
        THREE("3"),
        FOUR("4"),
        FIVE("5"),
        SIX("6"),
        SEVEN("7"),
        EIGHT("8"),
        NINE("9"),
        DECIMAL("."),
        ADD("+"),
        SUBTRACT("-"),
        DIVIDE("/"),
        MULTIPLY("*"),
        EQUALS("="),
        NUMBER_SIGN("#"),
        LEFT_PAREN("("),
        RIGHT_PAREN(")"),
        CLEAR("C");

        private final String toStringValue;

        Key(String toStringValue)
        {
            assert toStringValue.length() == 1;
            this.toStringValue = toStringValue;
        }

        /**
         * Returns the {@code char} value of this key. This is the value that should be input
         * into {@link ttaomae.timecalc.util.ExpressionFormatter#inputCharacter(char)
         * ExpressionFormatter#inputCharacter(char)}.
         */
        public char charValue()
        {
            return toStringValue.charAt(0);
        }

        /**
         * Returns the key corresponding to the specified {@code KeyCode}, or an empty optional if
         * none exists.
         */
        public static Optional<Key> fromKeyCode(KeyCode keyCode)
        {
            switch (keyCode) {
                case ESCAPE: return Optional.of(Key.CLEAR);
                case ENTER: return Optional.of(Key.EQUALS);
            }

            return Optional.empty();
        }

        /**
         * Returns the key corresponding to the specified character (as returned by {@link
         * javafx.scene.input.KeyEvent#getCharacter() KeyEvent#getCharacter()}), or an empty
         * optional if none exists.
         */
        public static Optional<Key> fromCharacter(String character)
        {
            switch (character) {
                case "0": return Optional.of(Key.ZERO);
                case "1": return Optional.of(Key.ONE);
                case "2": return Optional.of(Key.TWO);
                case "3": return Optional.of(Key.THREE);
                case "4": return Optional.of(Key.FOUR);
                case "5": return Optional.of(Key.FIVE);
                case "6": return Optional.of(Key.SIX);
                case "7": return Optional.of(Key.SEVEN);
                case "8": return Optional.of(Key.EIGHT);
                case "9": return Optional.of(Key.NINE);
                case ".": return Optional.of(Key.DECIMAL);
                case "+": return Optional.of(Key.ADD);
                case "-": return Optional.of(Key.SUBTRACT);
                case "*": return Optional.of(Key.MULTIPLY);
                case "/": return Optional.of(Key.DIVIDE);
                case "#": case "n": case "N": return Optional.of(Key.NUMBER_SIGN);
                case "(": return Optional.of(Key.LEFT_PAREN);
                case ")": return Optional.of(Key.RIGHT_PAREN);
                case "=": return Optional.of(Key.EQUALS);
                default: return Optional.empty();
            }
        }

        @Override
        public String toString()
        {
            return toStringValue;
        }
    }

    public Keypad()
    {
        getStyleClass().setAll("keypad");

        eventHandlers = new EnumMap<>(Key.class);
        for (Key key : Key.values()) {
            eventHandlers.put(key, new SimpleObjectProperty<>());
        }
    }

    private final EnumMap<Key, ObjectProperty<EventHandler<ActionEvent>>> eventHandlers;

    public ObjectProperty<EventHandler<ActionEvent>> onActionProperty(Key key)
    {
        return eventHandlers.get(key);
    }

    public EventHandler<ActionEvent> getOnAction(Key key)
    {
        return eventHandlers.get(key).get();
    }

    public void setOnAction(Key key, EventHandler<ActionEvent> eventHandler)
    {
        eventHandlers.get(key).set(eventHandler);
    }

    @Override
    protected Skin<?> createDefaultSkin()
    {
        return new KeypadSkin(this);
    }

    private class KeypadSkin extends SkinBase<Keypad>
    {
        private KeypadSkin(Keypad keypad)
        {
            super(keypad);

            var buttonGrid = new GridPane();
            buttonGrid.getStyleClass().setAll("keys");

            buttonGrid.add(newButton(Key.LEFT_PAREN),  0, 0);
            buttonGrid.add(newButton(Key.RIGHT_PAREN), 1, 0);
            buttonGrid.add(newButton(Key.CLEAR),       2, 0);
            buttonGrid.add(newButton(Key.DIVIDE),      3, 0);

            buttonGrid.add(newButton(Key.SEVEN),       0, 1);
            buttonGrid.add(newButton(Key.EIGHT),       1, 1);
            buttonGrid.add(newButton(Key.NINE),        2, 1);
            buttonGrid.add(newButton(Key.MULTIPLY),    3, 1);

            buttonGrid.add(newButton(Key.FOUR),        0, 2);
            buttonGrid.add(newButton(Key.FIVE),        1, 2);
            buttonGrid.add(newButton(Key.SIX),         2, 2);
            buttonGrid.add(newButton(Key.SUBTRACT),    3, 2);

            buttonGrid.add(newButton(Key.ONE),         0, 3);
            buttonGrid.add(newButton(Key.TWO),         1, 3);
            buttonGrid.add(newButton(Key.THREE),       2, 3);
            buttonGrid.add(newButton(Key.ADD),         3, 3);

            buttonGrid.add(newButton(Key.NUMBER_SIGN), 0, 4);
            buttonGrid.add(newButton(Key.ZERO),        1, 4);
            buttonGrid.add(newButton(Key.DECIMAL),     2, 4);
            buttonGrid.add(newButton(Key.EQUALS),      3, 4);

            getChildren().add(buttonGrid);

            for (var child : buttonGrid.getChildren()) {
                GridPane.setFillHeight(child, true);
                GridPane.setFillWidth(child, true);
                GridPane.setHgrow(child, Priority.ALWAYS);
                GridPane.setVgrow(child, Priority.ALWAYS);
            }
        }

        private Button newButton(Key key)
        {
            var button = new Button(key.toString());
            button.onActionProperty().bind(getSkinnable().onActionProperty(key));
            return button;
        }
    }
}
