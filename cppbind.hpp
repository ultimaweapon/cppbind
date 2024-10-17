// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef CPPBIND_HPP_INCLUDED
#define CPPBIND_HPP_INCLUDED

#include <stddef.h>

#define CPPBIND_CLASS(n) \
    template<> const size_t cppbind::type_info<n>::size = sizeof(n); \
    template<> const size_t cppbind::type_info<n>::align = alignof(n)

namespace cppbind {
    template<typename T>
    struct type_info {
        static const size_t size;
        static const size_t align;
    };
}

#endif // CPPBIND_HPP_INCLUDED
