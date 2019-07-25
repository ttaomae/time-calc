package ttaomae.timecalc;

import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.stage.Stage;

import java.io.IOException;

public class TimeCalculator extends Application
{
    @Override
    public void start(Stage stage) throws IOException
    {
        stage.setTitle("Time Calculator");
        var loader = new FXMLLoader(TimeCalculator.class.getResource("TimeCalculator.fxml"));
        stage.setScene(new Scene(loader.load()));
        stage.show();
    }

    public static void main(String[] args)
    {
        launch();
    }
}

