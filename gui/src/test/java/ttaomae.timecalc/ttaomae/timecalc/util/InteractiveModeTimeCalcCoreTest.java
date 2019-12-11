package ttaomae.timecalc.util;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.nio.file.Paths;

public class InteractiveModeTimeCalcCoreTest
{
    @Test
    public void testConstructorIllegalArguments()
    {
        try {
            new InteractiveModeTimeCalcCore(null);
            Assertions.fail();
        } catch (NullPointerException expected) {}

        try {
            new InteractiveModeTimeCalcCore(Paths.get("."));
            Assertions.fail();
        } catch (IllegalArgumentException expected) {}
    }
}
