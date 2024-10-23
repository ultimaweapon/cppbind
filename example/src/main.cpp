#include "../../cppbind.hpp"

#include <string>

class class1 {
public:
    class1();
    class1(const char *v1);
    ~class1();

    const char *value() const;
protected:
    std::string v1;
};

CPPBIND_CLASS(class1);

class1::class1()
{
}

class1::class1(const char *v1) : v1(v1)
{
}

class1::~class1()
{
}

const char *class1::value() const
{
    return v1.c_str();
}
