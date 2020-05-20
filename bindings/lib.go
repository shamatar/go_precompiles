package main

import (
	"C"
	"unsafe"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/vm"
)

//export run_precompile
func run_precompile(op C.char, i *C.char, iLen uint32) C.int {
	iBuff := C.GoBytes(unsafe.Pointer(i), C.int(iLen))

	precompilesList := vm.PrecompiledContractsIstanbul
	addressSingleByte := byte(op)
	address := common.BytesToAddress([]byte{addressSingleByte})

	precompile := precompilesList[address]

	_, _ = precompile.Run(iBuff)

	return 0
}

func main() {}
