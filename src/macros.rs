pub(crate) static CMD_LIST: &'static str = "
mkdir\t<OPERAND>\tCreate directory with name <OPERAND> \n
cd\t<OPERAND>\tChange directory to <OPERAND> \n
rmdir\t<OPERAND>\tRecursively remove directors from <OPERAND> \n
make\t<OPERAND>\tCreate file with name <OPERAND> \n
rm\t<OPERAND>\tRemove file with name <OPERAND> \n
view\t<OPERAND>\tShow contents file with name <OPERAND> \n
edit\t<OPERAND>\tEdit file with name <OPERAND>\t[WIP]\n
pwd\t\t\tShows present working directory \n
ls\t<OPTION>\tLists directories and files in the directory with name <OPTION> \n
history\t\t\tShows previously entered commands\n
quit\t\t\tExits the program\n
";
