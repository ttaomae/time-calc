package ttaomae.timecalc;

public class TimeFormatter
{
    private State currentState;
    private long hours;
    private int minutes;
    private int seconds;
    private StringBuilder fraction;

    public TimeFormatter()
    {
        this.fraction = new StringBuilder();
        clear();
    }

    public void clear()
    {
        this.currentState = State.SECONDS;
        this.hours = 0;
        this.minutes = 0;
        this.seconds = 0;
        this.fraction.setLength(0);
    }

    public String inputCharacter(char ch)
    {
        switch (this.currentState) {
            case SECONDS: {
                switch (ch) {
                    case '.': this.currentState = State.FRACTION; break;
                    case '0': this.appendSecond(0); break;
                    case '1': this.appendSecond(1); break;
                    case '2': this.appendSecond(2); break;
                    case '3': this.appendSecond(3); break;
                    case '4': this.appendSecond(4); break;
                    case '5': this.appendSecond(5); break;
                    case '6': this.appendSecond(6); break;
                    case '7': this.appendSecond(7); break;
                    case '8': this.appendSecond(8); break;
                    case '9': this.appendSecond(9); break;
                }
                break;
            }
            case FRACTION: {
                if (ch >= '0' && ch <= '9') {
                    this.appendFraction(ch);
                }
                break;
            }
        }

        return toString();
    }

    private void appendSecond(int digit)
    {
        assert digit >= 0 && digit <= 9 : "digit must be between 0 and 9.";

        // Shift left, then add upper digit of minutes.
        this.hours = (this.hours * 10) + (this.minutes / 10);

        // Shift left and truncate, then add upper digit of seconds.
        this.minutes = ((this.minutes * 10) % 100) + (this.seconds / 10);

        // Shift left and truncate, then add new digit.
        this.seconds = ((this.seconds * 10) % 100) + digit;
    }

    private void appendFraction(char digit)
    {
        assert digit >= '0' && digit <= '9' : "digit must be between 0 and 9.";

        // Do not add more than 9 digits.
        if (this.fraction.length() < 9) {
            this.fraction.append(digit);
        }
    }

    @Override
    public String toString() {
        switch (this.currentState) {
            case SECONDS:
                return String.format("%d:%02d:%02d", this.hours, this.minutes, this.seconds);
            case FRACTION:
                if (this.fraction.length() == 0) {
                    return String.format("%d:%02d:%02d.0", this.hours, this.minutes, this.seconds);
                }
                else {
                    return String.format("%d:%02d:%02d.%s",
                            this.hours, this.minutes, this.seconds, this.fraction);
                }
            default:
                throw new IllegalStateException("Unknown state.");
        }
    }

    private enum State
    {
        /** Currently appending seconds. */
        SECONDS,
        /** Currently appending fractional seconds. */
        FRACTION
    }
}
