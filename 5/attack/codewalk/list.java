import java.util.*;

public class list {
    public static void main(String args[]) {

        // Create an ArrayList `grades`.
        ArrayList<String> grades = new ArrayList<>(Arrays.asList(
            "A", "B", "C", "D", "E", "F"));

        // Print the list.
        Iterator<String> itr1 = grades.iterator();
        while(itr1.hasNext()) {
            System.out.print(itr1.next() + " ");
        }
        System.out.println();

        // Mutate the underlying list.
        ListIterator<String> listIter = grades.listIterator();
        while(listIter.hasNext()) {
            listIter.set(listIter.next() + "+");
        }

        // Print the list (again.).
        Iterator<String> itr2 = grades.iterator();
        while(itr2.hasNext()) {
            System.out.print(itr2.next() + " ");
        }
        System.out.println();

        // Print the list iterator backwards, we're already at the end.
        while(listIter.hasPrevious()) {
            System.out.print(listIter.previous() + " ");
        }
        System.out.println();
    }
}
