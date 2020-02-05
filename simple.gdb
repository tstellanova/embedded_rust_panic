# target extended-remote :50000

break panic
break DefaultHandler
break HardFault
set backtrace limit 32
set print asm-demangle on
set print pretty on
#break main

# setup ITM monitoring
# monitor tpiu config internal itm.txt uart off 8000000
# monitor tpiu config external uart off 480000000 2000000

# monitor itm port 0 on
