define internal void @Microsoft__Quantum__Testing__QIR__Main__body() {
entry:
  %0 = load i2, i2* @PauliI, align 1
  %1 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i2, i64 }* getelementptr ({ i2, i64 }, { i2, i64 }* null, i32 1) to i64))
  %2 = bitcast %Tuple* %1 to { i2, i64 }*
  %3 = getelementptr inbounds { i2, i64 }, { i2, i64 }* %2, i32 0, i32 0
  %4 = getelementptr inbounds { i2, i64 }, { i2, i64 }* %2, i32 0, i32 1
  store i2 %0, i2* %3, align 1
  store i64 0, i64* %4, align 4
  %5 = call { { i2, i64 }*, double }* @Microsoft__Quantum__Testing__QIR__TestType__body({ i2, i64 }* %2, double 0.000000e+00)
  %6 = call { %Tuple* }* @Microsoft__Quantum__Testing__QIR__Foo__body()
  %7 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %8 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %7, i64 0)
  %9 = bitcast i8* %8 to { %Tuple* }**
  store { %Tuple* }* %6, { %Tuple* }** %9, align 8
  %10 = call { i64 }* @Microsoft__Quantum__Testing__QIR__Bar__body(i64 1)
  %11 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %12 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %11, i64 0)
  %13 = bitcast i8* %12 to { i64 }**
  store { i64 }* %10, { i64 }** %13, align 8
  %14 = getelementptr inbounds { { i2, i64 }*, double }, { { i2, i64 }*, double }* %5, i32 0, i32 0
  %15 = load { i2, i64 }*, { i2, i64 }** %14, align 8
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %1, i32 -1)
  %16 = bitcast { i2, i64 }* %15 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %16, i32 -1)
  %17 = bitcast { { i2, i64 }*, double }* %5 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %17, i32 -1)
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %18 = phi i64 [ 0, %entry ], [ %24, %exiting__1 ]
  %19 = icmp sle i64 %18, 0
  br i1 %19, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %20 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %7, i64 %18)
  %21 = bitcast i8* %20 to { %Tuple* }**
  %22 = load { %Tuple* }*, { %Tuple* }** %21, align 8
  %23 = bitcast { %Tuple* }* %22 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %23, i32 -1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %24 = add i64 %18, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_reference_count(%Array* %7, i32 -1)
  br label %header__2

header__2:                                        ; preds = %exiting__2, %exit__1
  %25 = phi i64 [ 0, %exit__1 ], [ %31, %exiting__2 ]
  %26 = icmp sle i64 %25, 0
  br i1 %26, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %27 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %11, i64 %25)
  %28 = bitcast i8* %27 to { i64 }**
  %29 = load { i64 }*, { i64 }** %28, align 8
  %30 = bitcast { i64 }* %29 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %30, i32 -1)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %31 = add i64 %25, 1
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__array_update_reference_count(%Array* %11, i32 -1)
  ret void
}
