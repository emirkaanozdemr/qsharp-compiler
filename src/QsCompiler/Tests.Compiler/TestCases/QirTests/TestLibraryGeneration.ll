define void @Microsoft__Quantum__Testing__QIR__Baz__body(%Qubit* %q) {
entry:
  call void @__quantum__qis__h__body(%Qubit* %q)
  call void @__quantum__qis__t__body(%Qubit* %q)
  ret void
}

define void @Microsoft__Quantum__Testing__QIR__Baz__adj(%Qubit* %q) {
entry:
  call void @__quantum__qis__t__adj(%Qubit* %q)
  call void @__quantum__qis__h__body(%Qubit* %q)
  ret void
}

define i1 @Microsoft__Quantum__Testing__QIR__Foo__body(i64 %c1, i1 %c2) {
entry:
  %0 = icmp sgt i64 %c1, 0
  %1 = and i1 %0, %c2
  %2 = call i1 @Microsoft__Quantum__Testing__QIR_____GUID___Bar__body(i1 %1)
  ret i1 %2
}

define internal i1 @Microsoft__Quantum__Testing__QIR_____GUID___Bar__body(i1 %a1) {
entry:
  ret i1 %a1
}