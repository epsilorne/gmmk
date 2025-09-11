#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <hidapi/hidapi.h>

#define MAX_BUF 64
#define VID 0x320f
#define PID 0x5064

/**
 * Looks for the first device associated with the keyboard with an interface
 * number of 1, which corresponds to wIndex: 1 (in the USBHID packet).
 */
int init(hid_device **handle) {
  struct hid_device_info *dev, *cur_dev;
  dev = hid_enumerate(VID, PID);
  cur_dev = dev;

  while (cur_dev) {
    if (cur_dev->interface_number == 1)
      break;
    cur_dev = cur_dev -> next;
  }

  if (cur_dev) {
    *handle = hid_open_path(cur_dev->path);
    return 0;
  }
  return -1;
}

/**
 * Turn on the keyboard's LEDs by sending the right packets.
 */
int led_on(hid_device *handle, unsigned char *buf) {
  int res;

  // initial packet
  buf[0] = 0x04;
  buf[1] = 0x01;
  buf[2] = 0x00;
  buf[3] = 0x01;
  res = hid_send_output_report(handle, buf, MAX_BUF);
  if (res < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  memset(buf, 0, MAX_BUF);

  // packet to turn onled
  buf[0] = 0x04;
  buf[1] = 0x0c;
  buf[2] = 0x00;
  buf[3] = 0x06;
  buf[4] = 0x01;
  buf[5] = 0x04;
  buf[6] = 0x00;
  buf[7] = 0x00;
  buf[8] = 0x01;
  res = hid_send_output_report(handle, buf, MAX_BUF);
  if (res < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  memset(buf, 0, MAX_BUF);

  // end packet
  buf[0] = 0x04;
  buf[1] = 0x02;
  buf[2] = 0x00;
  buf[3] = 0x02;
  res = hid_send_output_report(handle, buf, MAX_BUF);
  if (res < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  memset(buf, 0, MAX_BUF);

  return 0;
}

/**
 * Turn on the keyboard's LEDs by sending the right packets.
 */
int led_off(hid_device *handle, unsigned char *buf) {
  int res;

  // initial packet
  buf[0] = 0x04;
  buf[1] = 0x01;
  buf[2] = 0x00;
  buf[3] = 0x01;
  res = hid_send_output_report(handle, buf, MAX_BUF);
  if (res < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  memset(buf, 0, MAX_BUF);

  // packet to turn off led
  buf[0] = 0x04;
  buf[1] = 0x0b;
  buf[2] = 0x00;
  buf[3] = 0x06;
  buf[4] = 0x01;
  buf[5] = 0x04;
  res = hid_send_output_report(handle, buf, MAX_BUF);
  if (res < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  memset(buf, 0, MAX_BUF);

  // end packet
  buf[0] = 0x04;
  buf[1] = 0x02;
  buf[2] = 0x00;
  buf[3] = 0x02;
  res = hid_send_output_report(handle, buf, MAX_BUF);
  if (res < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  memset(buf, 0, MAX_BUF);

  return 0;
}

int main(int argc, char **argv) {
  int res;
  unsigned char buf[64] = {0};
  hid_device *handle;
  
  res = hid_init();

  if (init(&handle)) {
    fprintf(stderr, "could not init keyboard\n");
    hid_exit();
    exit(EXIT_FAILURE);
  }

  // comment one or the other
  led_on(handle, buf);
  // led_off(handle, buf);

  hid_close(handle);
  hid_exit();

  exit(EXIT_SUCCESS);
}
