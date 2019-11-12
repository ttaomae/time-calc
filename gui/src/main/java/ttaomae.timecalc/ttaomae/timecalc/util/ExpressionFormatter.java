package ttaomae.timecalc.util;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class ExpressionFormatter
{
    private final ValueFormatter valueFormatter;
    private final List<Token> tokens;
    private int nUnclosedParentheses;

    public ExpressionFormatter()
    {
        valueFormatter = new ValueFormatter();
        tokens = new ArrayList<>();
        nUnclosedParentheses = 0;
    }

    public void clear()
    {
        valueFormatter.clear();
        tokens.clear();
        nUnclosedParentheses = 0;
    }

    public String inputCharacter(char ch)
    {
        if (charIsDigit(ch) || ch == '.' || ch == '#') {
            inputDigit(ch);
        }
        else if (charIsOperator(ch)) {
            inputOperator(ch);
        }
        else if (charIsParenthesis(ch)) {
            inputParenthesis(ch);
        }

        return toString();
    }

    private void inputDigit(char ch)
    {
        assert charIsDigit(ch) || ch == '.' || ch == '#';

        // If empty, pick an arbitrary operator so that empty is handled the same as operators.
        Token lastToken = getLastToken().orElse(Token.operator('+'));

        // If previous token is empty or operator ...
        if (lastToken.type == Token.Type.OPERATOR || lastToken.stringValue.equals("(")) {
            // ... add a new value token.
            valueFormatter.clear();
            valueFormatter.inputCharacter(ch);
            tokens.add(Token.value(valueFormatter.toString()));
        }
        // If previous token is time or number ...
        else if (lastToken.type == Token.Type.VALUE) {
            // ... update last token.
            valueFormatter.inputCharacter(ch);
            setLastToken(Token.value(valueFormatter.toString()));
        }
    }

    private void inputOperator(char ch)
    {
        assert charIsOperator(ch);

        getLastToken().ifPresent(token -> {
            // Operators can only follow a value or closing parenthesis.
            if (token.type == Token.Type.VALUE || token.stringValue.equals(")")) {
                tokens.add(Token.operator(ch));
            }
        });
    }

    private void inputParenthesis(char ch)
    {
        assert charIsParenthesis(ch);

        if (ch == '(') {
            var lastToken = getLastToken();
            // Open parentheses can start an expression or follow an operator or open parenthesis.
            if (lastToken.isEmpty() || lastToken.get().type == Token.Type.OPERATOR
                    || lastToken.get().stringValue.equals("(")) {
                tokens.add(Token.parenthesis(ch));
                nUnclosedParentheses++;
            }
        }
        else if (ch == ')') {
            // Closing parenthesis only allowed if there are unclosed parentheses.
            if (nUnclosedParentheses > 0) {
                var optionalLastToken = getLastToken();
                // If there are unclosed parentheses, then there must
                // at least be a previous parenthesis token.
                assert optionalLastToken.isPresent();
                var lastToken = optionalLastToken.get();

                // Cannot input closing parenthesis after operator or opening parenthesis.
                if (lastToken.type != Token.Type.OPERATOR && !lastToken.stringValue.equals("(")) {
                    tokens.add(Token.parenthesis(ch));
                    nUnclosedParentheses--;
                }
            }
        }
    }

    private Optional<Token> getLastToken()
    {
        if (tokens.size() == 0) {
            return Optional.empty();
        }
        else {
            return Optional.of(tokens.get(tokens.size() - 1));
        }
    }

    private void setLastToken(Token token)
    {
        assert tokens.size() > 0;

        int lastIndex = tokens.size() - 1;
        tokens.set(lastIndex, token);
    }

    @Override
    public String toString()
    {
        var result = new StringBuilder();

        var tokenIter = tokens.listIterator();
        while (tokenIter.hasNext()) {
            var currentToken = tokenIter.next();
            result.append(currentToken.stringValue);

            // Append space, if necessary.

            // Never append space if there are no more tokens.
            if (tokenIter.hasNext()) {
                var nextToken = tokenIter.next();
                // Append space after operators, values, and closing parenthesis ...
                if ((currentToken.type == Token.Type.OPERATOR
                        || currentToken.type == Token.Type.VALUE
                        || currentToken.stringValue.equals(")"))
                        // ... unless the next token is a closing parenthesis.
                        && !nextToken.stringValue.equals(")")) {
                    result.append(' ');
                }
                // Go back to current token.
                tokenIter.previous();
            }
        }
        return result.toString();
    }

    private static boolean charIsDigit(char ch)
    {
        return ch >= '0' && ch <= '9';
    }

    private static boolean charIsOperator(char ch)
    {
        return ch == '+' || ch == '-' || ch == '*' || ch == '/';
    }

    private static boolean charIsParenthesis(char ch)
    {
        return ch == '(' || ch == ')';
    }

    private static class Token
    {
        private enum Type
        {
            VALUE, OPERATOR, PARENTHESIS
        }

        private final Type type;
        private final String stringValue;

        private Token(Type type, String value)
        {
            this.type = type;
            this.stringValue = value;
        }

        private static Token value(String value)
        {
            return new Token(Type.VALUE, value);
        }

        private static Token operator(char value)
        {
            return new Token(Type.OPERATOR, String.valueOf(value));
        }

        private static Token parenthesis(char value)
        {
            return new Token(Type.PARENTHESIS, String.valueOf(value));
        }
    }
}
