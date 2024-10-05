// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef CPPBIND_HPP_INCLUDED
#define CPPBIND_HPP_INCLUDED

#include <stddef.h>

namespace cppbind {
    template<typename T>
    struct type_info {
        static const size_t size;
    };
}

#endif // CPPBIND_HPP_INCLUDED
