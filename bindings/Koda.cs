using System;
using System.Runtime.InteropServices;

public class Koda {
    [DllImport("koda.dll", CharSet = CharSet.Ansi)]
    private static extern IntPtr koda_parse_to_json(string input);

    [DllImport("koda.dll")]
    private static extern void koda_free_string(IntPtr ptr);

    public static string Parse(string input) {
        IntPtr ptr = koda_parse_to_json(input);
        string result = Marshal.PtrToStringAnsi(ptr);
        koda_free_string(ptr);
        return result;
    }
}
