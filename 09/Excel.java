public class Excel {

    public static void main(String[] args) {
        long n = Long.parseLong(args[0]);
    	String result = rec(n, "");
    	System.out.println(result);
    }

    public static String rec(long n, String result) {
	if ( n <= 0) {
		return result;
	}
       
       	long index = (n -1) % 26;
        result =  (char) (index + 65) + result;
        return rec((n - index) / 26, result);
    }
}
