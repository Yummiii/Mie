#include <ps2dev.h>
PS2dev keyboard(8, 5);
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
  unsigned char leds;
  if(keyboard.keyboard_handle(&leds)) {
    //Serial.print('LEDS');
    //Serial.print(leds, HEX);
    digitalWrite(LED_BUILTIN, leds);
  }
  if (Serial.available() > 0)
  {
    Serial.readBytes(buf, 3);
    if(buf[0] == 0) {
        keyboard.keyboard_release(buf[2]);
    } else {
        keyboard.keyboard_press(buf[2]);
    }
  }
}