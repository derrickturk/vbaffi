Option Explicit

Public Declare PtrSafe Function sum_values Lib "vbaffi.dll" ( _
  ByRef ex As ExampleUDT) As Double

Public Declare PtrSafe Function hypersum_values Lib "vbaffi.dll" ( _
  ByRef ex() As ExampleUDT) As Double

Public Declare PtrSafe Sub alter_values Lib "vbaffi.dll" ( _
  ByRef ex As ExampleUDT)

Public Declare PtrSafe Function make_array Lib "vbaffi.dll" () _
  As Long()

Public Type ExampleUDT
    Magic As Long
    Name As String
    Values() As Double
End Type

Public Sub DoItTryIt()
    Dim vals() As Double
    ReDim vals(1 To 10)
    Dim i As Long
    For i = 1 To 10
        vals(i) = 3.7 * i
    Next i

    Dim ex As ExampleUDT
    ex.Magic = &HDEADBEEF
    ex.Name = "DeadBeef"
    ex.Values = vals
    
    Debug.Print Hex(VarPtr(ex))

    ChDir ThisWorkbook.Path
    Debug.Print sum_values(ex)

    Dim exs(1 To 3) As ExampleUDT
    For i = 1 To 3
        exs(i) = ex
    Next i

    Debug.Print hypersum_values(exs)

    alter_values ex

    For i = 1 To 10
        Debug.Print ex.Values(i)
    Next i

    Dim xs() As Long
    xs = make_array()
    For i = LBound(xs) To UBound(xs)
        Debug.Print xs(i)
    Next i
End Sub
