jjc

# Ring Buffer | Circular buffer | Circular queue | Cyclic buffer
## Other implementations: 
- C++: https://www.boost.org/doc/libs/1_61_0/doc/html/circular_buffer.html

## Related Articles
  - https://en.wikipedia.org/wiki/Circular_buffer
  - https://embedjournal.com/implementing-circular-buffer-embedded-c/
  - https://embeddedartistry.com/blog/2017/05/17/creating-a-circular-buffer-in-c-and-c/
  - https://www.embedded.com/ring-buffer-basics/
  - https://www.sciencedirect.com/topics/engineering/circular-buffer
  - https://dzone.com/articles/ring-buffer-a-data-structure-behind-disruptor
  - https://ferrous-systems.com/blog/lock-free-ring-buffer/

## Use in code:
- None

## Typical use case: 
- None

## Possible optimization and design changes

Size optimization<br/>
Pass capasity by const generic if we know exact size and do not own memory

Size optimization<br/>
Template type for ```last```,```first``` or ```len``` field for known in advance buffer max size
Knowing the maximum size, we can use more economical types (It can also argument for using offset instead of pointers)

Size optimization<br/>
Now the buffer implimentation uses the ```len``` field to track its filling. 
We can use ```first``` and ```last``` field to calculate len of buffer.<br/>
Here presented possible state of buffer lenght
- empty invariant: 
    ```first == last```
- one item capacity:
    This state should be handlend directly. We can just use one of the fields (first or last) like bool.
- one item recorded to front or back: 
    To solve this problem, we can use various known in advance invalid states.
    - invalid addresses (outside for the size of allocated memory) or if we use offset then negative values
- partially filled: 
 - fisr points to the item at the end
 - last points to the item at the front
- full invariant: 
    ```last == fist - 1```
    To save the invariant, we must move the pointers synchronously

Size optimization<br/>
If we use offsets and know max size. A good solution is to use bit fields. 

Size optimization<br/>
Offsets first and last always less by one than capacyty so we can use it last value how invariant.

Design changes<br/>
Full buffer behavior<br/>
Now  buffer overwrites incomming data. In general, someone may need a different behavior, for example: drop incoming data, return error or etc. 
