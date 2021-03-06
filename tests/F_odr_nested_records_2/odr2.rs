#pragma version(1)

#pragma rs java_package_name(com.android.rs.test)

// Mismatching Inner field names
struct Inner {
  uint32_t y;
};

struct Outer {
  struct Inner current;
};

extern uint32_t uint32_ret;

extern struct Outer *outer;

void outer_y(void) {
  uint32_ret = outer->current.y;
}

