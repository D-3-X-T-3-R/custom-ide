pub(crate) static CMD_LIST: &'static str = "
mkdir\t<OPERAND>\t\tCreate directory with name <OPERAND> \n
cd\t<OPERAND>\t\tChange directory to <OPERAND> \n
rmdir\t<OPERAND>\t\tRecursively remove directors from <OPERAND> \n
make\t<OPERAND>\t\tCreate file with name <OPERAND> \n
rm\t<OPERAND>\t\tRemove file with name <OPERAND> \n
mv\t<OPERAND> <OPERAND>\tMoves/Renames file/folder <OPERAND> to file/folder <OPERAND>\n
view\t<OPERAND>\t\tShow contents file with name <OPERAND> \n
edit\t<OPERAND>\t\tEdit file with name <OPERAND> [WIP]\n
cp\t<OPERAND> <OPERAND>\tCopies file/folders <OPERAND> to file/folders <OPERAND>\n
pwd\t\t\t\tShows present working directory \n
ls\t<OPTION>\t\tLists directories and files in the directory with name <OPTION> \n
history\t\t\t\tShows previously entered commands\n
clear\t\t\t\tClears the terminal\n
quit\t\t\t\tExits the program\n
";
