echo off

goto(){
dx serve --hot-reload --port 4040
uname -o
}

goto $@
exit

:(){
dx serve --hot-reload --port 4040
echo %OS%
exit