module ttaomae.timecalc {
    exports ttaomae.timecalc;

    opens ttaomae.timecalc to javafx.fxml;
    opens ttaomae.timecalc.control to javafx.fxml;

    requires ttaomae.timecalc.core;
    requires java.logging;
    requires javafx.base;
    requires javafx.controls;
    requires javafx.fxml;
}
