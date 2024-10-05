#include "../../cppbind.hpp"

#include <string>

class class1 {
public:
    class1()
    {
    }

    class1(const char *v1) : v1(v1)
    {
    }
protected:
    std::string v1;
};

template<>
const size_t cppbind::type_info<class1>::size = sizeof(class1);
