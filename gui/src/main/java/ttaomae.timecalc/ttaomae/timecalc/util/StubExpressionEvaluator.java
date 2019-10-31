package ttaomae.timecalc.util;

public class StubExpressionEvaluator implements ExpressionEvalutor {
    @Override
    public Result<String, String> evaluate(String expression) {
        if (expression.contains("0")) {
            return Result.success("success");
        }
        else {
            return Result.failure("failure");
        }
    }
}
