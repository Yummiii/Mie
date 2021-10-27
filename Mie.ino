#include <ps2dev.h>   
PS2dev keyboard(3,2);

void setup() {
  Serial.begin(9600);
  keyboard.keyboard_init();
  pinMode(LED_BUILTIN, OUTPUT);

}

byte buf[3];
void loop() {
  if (Serial.available() > 0) {
    Serial.readBytes(buf, 3);
    Serial.println(buf[0]); 
    Serial.println(buf[1]); 
    Serial.println(buf[2]);
    
    digitalWrite(LED_BUILTIN, HIGH);   // turn the LED on (HIGH is the voltage level)
    delay(100);                       // wait for a second
    digitalWrite(LED_BUILTIN, LOW);    // turn the LED off by making the voltage LOW
    delay(100); 
    
  }
}
