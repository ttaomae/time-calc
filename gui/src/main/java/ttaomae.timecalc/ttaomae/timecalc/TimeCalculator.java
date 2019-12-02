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
        var fxmlLoader = new FXMLLoader();
        fxmlLoader.setLocation(TimeCalculator.class.getResource("TimeCalculator.fxml"));

        stage.setTitle("Time Calculator");
        stage.setScene(new Scene(fxmlLoader.load()));
        stage.show();
        stage.setMinHeight(stage.getHeight());
        stage.setMinWidth(stage.getWidth());
    }

    public static void main(String[] args)
    {
        launch();
    }
}
