#include <ps2dev.h>
PS2dev keyboard(3, 2);
//amarelo clock (3), branco data (2)

void setup()
{
  Serial.begin(9600);
  keyboard.keyboard_init();
  pinMode(LED_BUILTIN, OUTPUT);
}

byte buf[3];
void loop()
{
  if (Serial.available() > 0)
  {
    Serial.readBytes(buf, 3);

    if (buf[0] == 0)
    {
      if (buf[1] == 0)
      {
        keyboard.keyboard_release(buf[3]);
      }
      else
      {
        keyboard.keyboard_release_special(buf[3]);
      }
    }
    else
    {
      if (buf[1] == 0)
      {
        keyboard.keyboard_press(buf[3]);
      }
      else
      {
        keyboard.keyboard_press_special(buf[3]);
      }
    }
    digitalWrite(LED_BUILTIN, HIGH); // turn the LED on (HIGH is the voltage level)
    delay(100);                      // wait for a second
    digitalWrite(LED_BUILTIN, LOW);  // turn the LED off by making the voltage LOW
    delay(100);
  }
}
