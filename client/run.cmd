echo off

goto(){
dx serve --hot-reload
uname -o
}

goto $@
exit

:(){
dx serve --hot-reload
echo %OS%
exit