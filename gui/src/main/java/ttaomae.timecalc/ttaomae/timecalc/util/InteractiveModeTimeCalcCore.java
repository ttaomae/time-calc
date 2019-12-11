package ttaomae.timecalc.util;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.lang.SuppressWarnings;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Objects;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.SynchronousQueue;
import java.util.concurrent.TimeUnit;
import java.util.logging.Logger;

/**
 * An expression evaluator which uses {@code time-calc/core| executable in interactive mode.
 * Expressions, terminated by line separators, are written to the stdin of the process. A result or
 * error is then written the process
 */
public final class InteractiveModeTimeCalcCore implements ExpressionEvalutor
{
    private static final Logger LOGGER =
            Logger.getLogger(InteractiveModeTimeCalcCore.class.getCanonicalName());
    /** Writes to the stdin of the time-calc process. */
    private final OutputStreamWriter timeCalcStdin;
    private final Process timeCalcProcess;

    private final BlockingQueue<Result<String, String>> resultQueue;

    @SuppressFBWarnings(value = "OS_OPEN_STREAM",
            justification = "This appears to be a false positive. stdout/stderr streams are closed "
                    + "implicitly by try-with-resources in fillQueue method.")
    public InteractiveModeTimeCalcCore(Path commandPath)
    {
        Objects.requireNonNull(commandPath, "commandPath must not be null.");
        if (!Files.isRegularFile(commandPath)) {
            throw new IllegalArgumentException("commandPath must refer to an existing file.");
        }

        try {
            var command = commandPath.toAbsolutePath().toString();
            timeCalcProcess = new ProcessBuilder(command).start();
        }
        catch (IOException e) {
            throw new IllegalStateException("Could not start time-calc process.", e);
        }

        timeCalcStdin = new OutputStreamWriter(timeCalcProcess.getOutputStream(), StandardCharsets.UTF_8);
        resultQueue = new SynchronousQueue<>();

        var timeCalcStdout = new BufferedReader(
                new InputStreamReader(timeCalcProcess.getInputStream(), StandardCharsets.UTF_8));
        var timeCalcStderr = new BufferedReader(
                new InputStreamReader(timeCalcProcess.getErrorStream(), StandardCharsets.UTF_8));

        ExecutorService standardStreamThreadPool = newStandardStreamThreadPool();
        standardStreamThreadPool.submit(
                () -> fillQueue(timeCalcStdout, StandardStream.STDOUT, resultQueue));
        standardStreamThreadPool.submit(
                () -> fillQueue(timeCalcStderr, StandardStream.STDERR, resultQueue));
    }

    @Override
    public Result<String, String> evaluate(String expression)
    {
        if (expression == null || expression.isBlank()) return Result.success("");

        synchronized (timeCalcStdin) {
            // Write expression to time-calc process, which evaluates expressions delimited by newlines.
            try {
                timeCalcStdin.write(expression + "\n");
                timeCalcStdin.flush();
            }
            catch (IOException e) {
                return Result.failure("Could not evaluate expression.");
            }

            // Assumptions:
            // * We are the only one with access to the process's stdin/out/err streams.
            // * The only time we write to the processes stdin is in this method.
            // * Since everything here is in a synchronized block, expressions will only be written
            //   one at a time.
            // * We consume all results so there is no "backlog" of old results.
            // Therefore, the next result is the result for the current expression.
            try {
                return resultQueue.take();
            }
            catch (InterruptedException e) {
                shutDown();
                return Result.failure("Shutting down.");
            }
        }
    }

    /**
     * Terminates time-calc interactive mode and stops threads reading from stdout/stderr.
     */
    public void shutDown() {
        try {
            // Close stream into stdin, which should end the process.
            timeCalcStdin.close();
            timeCalcProcess.waitFor();
        }
        catch (IOException e) {
            LOGGER.warning("Could not close stdin stream.");
        }
        catch (InterruptedException e) {
            LOGGER.warning("Interruped while waiting for time-calc process to complete.");
        }
    }

    private enum StandardStream { STDOUT, STDERR }
    private void fillQueue(BufferedReader reader, StandardStream streamType,
            BlockingQueue<Result<String, String>> queue)
    {
        try (reader) {
            String line;
            while ((line = reader.readLine()) != null) { // NOPMD
                switch (streamType) {
                    case STDOUT: queue.put(Result.success(line)); break;
                    case STDERR: queue.put(Result.failure(line)); break;
                    default: throw new IllegalStateException("Unknown standard stream: " + streamType);
                }
            }
        }
        // Thrown by reader.readLine()
        catch (IOException e) {
            LOGGER.warning(() -> "Could not read from " + streamType + " stream.");
        }
        // Thrown by queue.put
        catch (InterruptedException e) {
            LOGGER.info(() -> "Shutting down " + streamType + " thread.");
        }
    }

    /**
     * Creates a new thread pool for theads reading from the time-calc standard output streams
     * (stdout and stderr).
     */
    @SuppressWarnings("PMD.DoNotUseThreads")
    private static ExecutorService newStandardStreamThreadPool() {
        return Executors.newFixedThreadPool(2, runnable -> {
            Thread thread = new Thread(runnable);
            thread.setDaemon(true);
            thread.setName("time-calc-reader");
            return thread;
        });
    }
}
