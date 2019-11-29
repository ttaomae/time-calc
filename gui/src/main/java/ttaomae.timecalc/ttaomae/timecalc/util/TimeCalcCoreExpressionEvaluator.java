package ttaomae.timecalc.util;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Objects;
import java.util.Optional;
import java.util.OptionalInt;
import java.util.function.Function;

/**
 * An expression evaluator which calls the {@code time-calc/core} executable.
 */
public class TimeCalcCoreExpressionEvaluator implements ExpressionEvalutor
{
    private final String calcCommand;

    public TimeCalcCoreExpressionEvaluator(Path commandPath)
    {
        Objects.requireNonNull(commandPath, "commandPath must not be null.");
        if (!Files.isRegularFile(commandPath)) {
            throw new IllegalArgumentException("commandPath must refer to an existing file.");
        }
        calcCommand = commandPath.toAbsolutePath().toString();
    }

    @Override
    public Result<String, String> evaluate(String expression) {
        Process process = startProcess(expression)
                .orElseThrow(() -> new IllegalStateException("Could not start process."));

        int exitCode = waitForProcess(process)
                .orElseThrow(() -> new IllegalStateException("Process interrupted."));

        if (exitCode == 0) {
            String result = readProcessStream(process, Process::getInputStream)
                    .orElseThrow(() -> new IllegalStateException("Could not read process output."));
            return Result.success(result);
        }
        else {
            String error = readProcessStream(process, Process::getErrorStream)
                    .orElseThrow(() -> new IllegalStateException("Could not read process output."));
            return Result.failure(error);
        }
    }

    private Optional<Process> startProcess(String expression)
    {
        try {
            return Optional.of(new ProcessBuilder(calcCommand, expression).start());
        }
        catch (IOException e) {
            return Optional.empty();
        }
    }

    private OptionalInt waitForProcess(Process process)
    {
        try {
            return OptionalInt.of(process.waitFor());
        }
        catch (InterruptedException e) {
            return OptionalInt.empty();
        }
    }

    private Optional<String> readProcessStream(Process process,
            Function<Process, InputStream> inputStreamGetter)
    {
        try (var calcOutput = inputStreamGetter.apply(process)) {
            String result = new String(calcOutput.readAllBytes(), StandardCharsets.UTF_8);
            return Optional.of(result);
        }
        catch (IOException e) {
            return Optional.empty();
        }
    }
}
