package ttaomae.timecalc.core;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

/**
 * Empty class whose sole purpose is to allow resources to be loaded from this module.
 */
public final class TimeCalcLoader
{
    private static final Properties PROPERTIES;
    static {
        try (var in = TimeCalcLoader.class.getResourceAsStream("/time-calc.properties")) {
            PROPERTIES = new Properties();
            PROPERTIES.load(in);
        }
        catch (IOException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    // Prevent instantiation.
    private TimeCalcLoader() {}

    /**
     * Returns the name of the time-calc/core executable.
     */
    public static String getExecutableName()
    {
        return PROPERTIES.getProperty("time-calc.binary-name");
    }

    /**
     * Returns the time-calc/core executable as an input stream.
     */
    public static InputStream getExecutableAsStream()
    {
        return TimeCalcLoader.class.getResourceAsStream("/" + getExecutableName());
    }
}
