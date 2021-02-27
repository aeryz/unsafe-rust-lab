#include <iostream>

template <typename T, typename U> struct Foo {
  short count;
  T data1;
  U data2;

  Foo(short count, T data1, U data2)
      : count(count), data1(data1), data2(data2) {}
};

int main() {

    auto foo1 = Foo<short, int>(1, 1, 1);
    auto foo2 = Foo<int, short>(1, 1, 1);

    std::cout << sizeof(foo1) << ' ' << sizeof(foo2) << '\n';

    return 0;
}
