#shellcheck shell=sh
Describe 'Whitespace'
  It 'newlines can separate tokens'
    echo "5" >> $program
    echo "+" >> $program
    echo "STDOUT" >> $program
    echo ";" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '6'
    The status should be success
  End

  It 'Empty lines can separate tokens'
    echo "5" >> $program
    echo "\n\n\n" >> $program
    echo "-" >> $program
    echo "\n\n\n" >> $program
    echo "STDOUT" >> $program
    echo "\n\n\n" >> $program
    echo ";" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '4'
    The status should be success
  End

  It 'Multi-statement programs can span multiple lines'
    echo "5 +" >> $program
    echo "STDOUT;" >> $program
    echo "10 -" >> $program
    echo "STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The status should be success
    The first line of stdout should eq '6'
    The second line of stdout should eq '9'
  End

  It 'Operators can occur without whitespace'
    echo "5 +-+-+STDOUT+-+-+STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The status should be success
    The first line of stdout should eq '6'
    The second line of stdout should eq '7'
  End
End
