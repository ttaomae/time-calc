package ttaomae.timecalc.util;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.nio.file.Paths;

public class SingleExpressionTimeCalcCoreTest
{
    @Test
    public void testConstructorIllegalArguments()
    {
        try {
            new SingleExpressionTimeCalcCore(null);
            Assertions.fail();
        } catch (NullPointerException expected) {}

        try {
            new SingleExpressionTimeCalcCore(Paths.get("."));
            Assertions.fail();
        } catch (IllegalArgumentException expected) {}
    }
}
