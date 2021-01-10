#include <cerrno>

extern "C" int get_errno(){
    return errno;
}
