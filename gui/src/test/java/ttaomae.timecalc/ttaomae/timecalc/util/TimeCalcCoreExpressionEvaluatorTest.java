package ttaomae.timecalc.util;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.nio.file.Paths;

public class TimeCalcCoreExpressionEvaluatorTest
{
    @Test
    public void testConstructorIllegalArguments()
    {
        try {
            new TimeCalcCoreExpressionEvaluator(null);
            Assertions.fail();
        } catch (NullPointerException expected) {}

        try {
            new TimeCalcCoreExpressionEvaluator(Paths.get("."));
            Assertions.fail();
        } catch (IllegalArgumentException expected) {}
    }
}
