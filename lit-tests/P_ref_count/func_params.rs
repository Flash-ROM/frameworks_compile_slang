// RUN: %Slang %s
// RUN: %rs-filecheck-wrapper %s
// CHECK-NOT: call void @_Z11rsSetObjectP10rs_elementS_({{.*}} %.rs.param.a0, {{.*}})
// CHECK: call void @_Z11rsSetObjectP10rs_elementS_(%struct.rs_element{{.*}}* nonnull %.rs.param.a1, {{.*}})
// CHECK-NOT: call void @_Z11rsSetObjectP10rs_elementS_({{.*}} %.rs.param.a2, {{.*}})
// CHECK: call void @_Z13rsClearObjectP10rs_element(%struct.rs_element{{.*}}* nonnull %x)
// CHECK-NOT: call void @_Z13rsClearObjectP10rs_element({{.*}}* %.rs.param.a0)
// CHECK: call void @_Z13rsClearObjectP10rs_element(%struct.rs_element{{.*}}* {{.*}}%a1)
// CHECK-NOT: call void @_Z13rsClearObjectP10rs_element({{.*}}* %.rs.param.a2)

#pragma version(1)
#pragma rs java_package_name(ref_count)

static void bar(int a0, rs_element a1, float a2) {
  rs_element x = {0};
  a1 = x;
}

void entrypoint() {
  rs_element e = {0};
  bar(3, e, 2.718f);
  if (rsIsObject(e)) {
    rsDebug("good object", 0);
  }
}


