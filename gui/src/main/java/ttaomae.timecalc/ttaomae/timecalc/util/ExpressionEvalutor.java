package ttaomae.timecalc.util;

public interface ExpressionEvalutor {
    Result<String, String> evaluate(String expression);
}
