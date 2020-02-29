mod conc;

fn main() {
    conc::basic_example();
    conc::using_data_from_parent_thread();
    conc::using_channels();
    conc::using_mutexes();
}
