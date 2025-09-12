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
 * Sends an output report containing the bytes 'to_send' (of size 'n'). 'buf' is
 * a 64 byte buffer, while 'to_send' and 'n' are up to 63 bytes.
 */
int send_report(hid_device *handle, unsigned char *buf, unsigned char *to_send, size_t n) {
  int res;
  memset(buf, 0, MAX_BUF);
  memcpy(buf, to_send, n);
  if ((res = hid_send_output_report(handle, buf, MAX_BUF)) < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  return res;
}

/**
 * Turn on the keyboard's LEDs by sending the right packets.
 */
int led_on(hid_device *handle, unsigned char *buf) {
  if (send_report(handle, buf, (unsigned char[]){0x04, 0x01, 0x00, 0x01}, 4) < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  if (send_report(handle, buf, (unsigned char[]){0x04, 0x0c, 0x00, 0x06, 0x01, 0x04, 0x00, 0x00, 0x01}, 9) < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  if (send_report(handle, buf, (unsigned char[]){0x04, 0x02, 0x00, 0x02}, 4) < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  return 0;

  return 0;
}

/**
 * Turn off the keyboard's LEDs by sending the right packets.
 */
int led_off(hid_device *handle, unsigned char *buf) {
  if (send_report(handle, buf, (unsigned char[]){0x04, 0x01, 0x00, 0x01}, 4) < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  if (send_report(handle, buf, (unsigned char[]){0x04, 0x0b, 0x00, 0x06, 0x01, 0x04}, 6) < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  if (send_report(handle, buf, (unsigned char[]){0x04, 0x02, 0x00, 0x02}, 4) < 0) {
    fprintf(stderr, "failed to send output report\n");
    return -1;
  }
  return 0;
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "usage: driver <on/off>\n");
    exit(EXIT_FAILURE);
  }

  int res;
  unsigned char buf[64] = {0};
  hid_device *handle;
  
  res = hid_init();

  if (init(&handle) < 0) {
    fprintf(stderr, "could not init keyboard\n");
    hid_exit();
    exit(EXIT_FAILURE);
  }

  if (strcmp(argv[1], "on") == 0) {
    if (led_on(handle, buf) < 0) {
      fprintf(stderr, "failed to turn on led\n");
    }
  }
  else if (strcmp(argv[1], "off") == 0) {
    if (led_off(handle, buf) < 0) {
      fprintf(stderr, "failed to turn off led\n");
    }
  }

  hid_close(handle);
  hid_exit();

  exit(EXIT_SUCCESS);
}
