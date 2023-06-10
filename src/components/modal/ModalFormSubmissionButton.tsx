import { Button, Flex } from '@mantine/core';

interface ModalFormSubmissionButtonProps {
  confirmText?: string;
  cancelText?: string;
  onClose: () => void;
}

const ModalFormSubmissionButton = ({
  confirmText = 'ยืนยัน',
  cancelText = 'ยกเลิก',
  onClose,
}: ModalFormSubmissionButtonProps) => {
  return (
    <Flex justify="flex-end" gap="xs">
      <Button type="submit">{confirmText}</Button>
      <Button variant="outline" onClick={onClose}>
        {cancelText}
      </Button>
    </Flex>
  );
};
export default ModalFormSubmissionButton;
