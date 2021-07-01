use fltk::{
    app,
    browser::{Browser, BrowserType},
    prelude::{BrowserExt, GroupExt, WidgetBase, WidgetExt},
    window::Window,
};

fn main() {
    let app = app::App::default();
    let mut w = Window::default().with_size(900, 300);
    let mut b = Browser::new(10, 10, w.w() - 20, w.h() - 20, None);
    let widths = &[50, 50, 50, 70, 70, 40, 40, 70, 70, 50]; // widths for each column
    b.set_column_widths(widths);
    b.set_column_char('\t'); // tabs as column delimiters
    b.set_type(BrowserType::Multi);
    b.add("USER\tPID\t%CPU\t%MEM\tVSZ\tRSS\tTTY\tSTAT\tSTART\tTIME\tCOMMAND"); // lines of tab delimited data
    b.add("root\t2888\t0.0\t0.0\t1352\t0\ttty3\tSW\tAug15\t0:00\t@b@f/sbin/mingetty tty3");
    b.add("erco\t2889\t0.0\t13.0\t221352\t0\ttty3\tR\tAug15\t1:34\t@b@f/usr/local/bin/render a35 0004");
    b.add("uucp\t2892\t0.0\t0.0\t1352\t0\tttyS0\tSW\tAug15\t0:00\t@b@f/sbin/agetty -h 19200 ttyS0 vt100");
    b.add("root\t13115\t0.0\t0.0\t1352\t0\ttty2\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty2");
    b.add(
        "root\t13464\t0.0\t0.0\t1352\t0\ttty1\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty1 --noclear",
    );
    w.resizable(&b);
    w.end();
    w.show();
    app.run().unwrap();
}
