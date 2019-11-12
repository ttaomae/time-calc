package ttaomae.timecalc.util;

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
        if (ch == '#') {
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

    private static boolean charIsDigit(char ch)
    {
        return ch >= '0' && ch <= '9';
    }

    @Override
    public String toString() {
        var result = new StringBuilder();

        var nDigits = wholeInput.length();
        if (currentType == Type.TIME) {
            // Hours.
            if (nDigits >= 5) {
                result.append(wholeInput.substring(0, nDigits - 4))
                        .append(':');
            }
            // Two minute digits.
            if (nDigits >= 4) {
                result.append(wholeInput.substring(nDigits - 4, nDigits - 2))
                        .append(':');
            }
            // One minute digit.
            else if (nDigits == 3) {
                result.append('0')
                        .append(wholeInput.substring(nDigits - 3, nDigits - 2))
                        .append(':');
            }
            // Two digit seconds.
            if (nDigits >= 2) {
                result.append(wholeInput.substring(nDigits - 2, nDigits));
            }
            // One digit seconds.
            else if (nDigits == 1) {
                result.append(wholeInput);
            }

            // Zero if no whole input.
            if (nDigits == 0) {
                result.append('0');
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
            if (fractionInput.length() == 0) result.append('0');
            else result.append(fractionInput);
        }

        // Append 's' for time with seconds only.
        if (currentType == Type.TIME && nDigits <= 2) {
            result.append('s');
        }

        return result.toString();
    }

    private enum Type
    {
        TIME, NUMBER
    }

    private enum State
    {
        /** Currently appending to whole part. */
        WHOLE,
        /** Currently appending to fractional part. */
        FRACTION
    }
}
