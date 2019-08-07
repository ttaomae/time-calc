package ttaomae.timecalc;

public class ValueFormatter
{
    private Type currentType;
    private State currentState;
    private StringBuilder wholeInput;
    private StringBuilder fractionInput;

    public ValueFormatter()
    {
        this.currentType = Type.TIME;
        this.currentState = State.WHOLE;
        this.wholeInput = new StringBuilder();
        this.fractionInput = new StringBuilder();
    }

    public void clear()
    {
        this.currentType = Type.TIME;
        this.currentState = State.WHOLE;
        this.wholeInput.setLength(0);
        this.fractionInput.setLength(0);
    }

    public String inputCharacter(char ch)
    {
        if (ch == 'n') {
            toggleType();
        }
        else if (ch == '.' && this.currentState == State.WHOLE) {
            this.currentState = State.FRACTION;
        }
        else {
            appendDigit(ch);
        }

        return toString();
    }

    private void toggleType()
    {
        switch (this.currentType) {
            case TIME: this.currentType = Type.NUMBER; break;
            case NUMBER: this.currentType = Type.TIME; break;
        }
    }

    private void appendDigit(char ch)
    {
        if (charIsDigit(ch)) {
            switch (currentState) {
                case WHOLE:
                    // Skip leading zeros.
                    if (wholeInput.length() > 0 || ch != '0') wholeInput.append(ch);
                    break;
                case FRACTION:
                    if (fractionInput.length() < 9) fractionInput.append(ch);
                    break;
            }
        }
    }

    private boolean charIsDigit(char ch)
    {
        return ch >= '0' && ch <= '9';
    }

    @Override
    public String toString() {
        var result = new StringBuilder();

        if (currentType == Type.TIME) {
            var nDigits = wholeInput.length();
            // Construct h:mm:ss from input.
            // Must be at least 5 digits, so iterate from negative index if necessary.
            for (int i = Math.min(0, nDigits - 5); i < nDigits; i++) {
                // Add colon separator before last 2 and last 4 digits.
                if (nDigits - i == 2 || nDigits - i == 4)  {
                    result.append(':');
                }
                // Use '0' if we need extra digits.
                if (i < 0) result.append('0');
                else result.append(wholeInput.charAt(i));
            }
        }
        else if (currentType == Type.NUMBER) {
            if (wholeInput.length() == 0) result.append('0');
            else result.append(wholeInput);
        }

        // Fractional part is the same for times and numbers.
        if (currentState == State.FRACTION) {
            result.append('.');
            // If there is no digits input after decimal, add '0'.
            if (fractionInput.length() == 0) {
                result.append('0');
            }
            else {
                result.append(fractionInput);
            }
        }

        // Append 'n' suffix used to identify numbers.
        if (currentType == Type.NUMBER) result.append('n');

        return result.toString();
    }

    private enum Type
    {
        TIME, NUMBER
    }

    private enum State
    {
        /** Currently appending seconds. */
        WHOLE,
        /** Currently appending fractional seconds. */
        FRACTION
    }
}
