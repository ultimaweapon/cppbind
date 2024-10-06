// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef CPPBIND_HPP_INCLUDED
#define CPPBIND_HPP_INCLUDED

#include <stddef.h>

#define CPPBIND_CLASS(n) \
    template<> const size_t cppbind::type_info<n>::size = sizeof(n)

namespace cppbind {
    template<typename T>
    struct type_info {
        static const size_t size;
    };
}

#endif // CPPBIND_HPP_INCLUDED
