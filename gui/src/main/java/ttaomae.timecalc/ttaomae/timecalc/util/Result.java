package ttaomae.timecalc.util;

import java.util.Optional;

/**
 * A result which can be either a {@linkplain #success(Object) success} with a {@linkplain
 * #getValue() value} or a {@linkplain #failure(Object) failure} with an {@linkplain #getError()
 * error}.
 *
 * @param <T> the success value type
 * @param <E> the failure error type
 */
public final class Result<T, E>
{
    private final T value;
    private final E error;

    private Result(T value, E error)
    {
        assert value == null ^ error == null;
        this.value = value;
        this.error = error;
    }

    /**
     * Returns a new successful result with the specified value.
     *
     * @param value the result value
     * @param <T> the type of a successful result
     * @param <E> the type of a failure result
     * @return a new successful result
     */
    public static <T, E> Result<T, E> success(T value)
    {
        return new Result<>(value, null);
    }

    /**
     * Returns a new failure result with the specified value
     *
     * @param error the failure value
     * @param <T> the type of a successful result
     * @param <E> the type of a failure result
     * @return a new failure result
     */
    public static <T, E> Result<T, E> failure(E error)
    {
        return new Result<>(null, error);
    }

    /**
     * Returns {@code true} if the result represents a success, otherwise {@code false}.
     * @return whether or not the result represents a success
     */
    public boolean isSuccess()
    {
        return value != null;
    }

    /**
     * Returns the result value, or an empty optional if the result is a failure.
     * @return the result value, or an empty optional if the result is a failure.
     */
    public Optional<T> getValue()
    {
        return Optional.ofNullable(value);
    }

    /**
     * Returns {@code true} if the result represents a failure, otherwise {@code false}.
     * @return whether or not the result represents a failure
     */
    public boolean isFailure()
    {
        return error != null;
    }

    /**
     * Returns the error, or an empty optional if the result is a success.
     * @return the error, or an empty optional if the result is a success
     */
    public Optional<E> getError()
    {
        return Optional.ofNullable(error);
    }

    @Override
    public String toString()
    {
        assert value != null ^ error != null;
        if (value != null) return String.format("Success[%s]", value);
        return String.format("Failure[%s]", error);
    }
}
