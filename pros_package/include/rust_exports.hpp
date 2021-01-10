#ifndef PROS_PACKAGE_RUST_EXPORTS_HPP
#define PROS_PACKAGE_RUST_EXPORTS_HPP

extern "C"{
    void rust_initialize();
    void rust_disabled();
    void rust_competition_initialize();
    void rust_autonomous();
    void rust_opcontrol();
}

#endif //PROS_PACKAGE_RUST_EXPORTS_HPP
