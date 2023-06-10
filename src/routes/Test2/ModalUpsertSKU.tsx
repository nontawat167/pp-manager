import Modal from '@Components/modal/Modal';
import ModalFormSubmissionButton from '@Components/modal/ModalFormSubmissionButton';
import { Box } from '@mantine/core';
import {
  SKUUpsertFormProvider,
  SKUUpsertFormInput,
} from './SKUUpsertFormContext';
import SKUUpertFormFields from './SKUUpsertFormFields';

interface ModalUpsert {
  opened: boolean;
  close: () => void;
}

const ModalUpsertSKU = ({ opened, close }: ModalUpsert) => {
  const handleSubmit = (data: SKUUpsertFormInput) => {
    console.log(data);
    close();
  };

  return (
    <Modal opened={opened} onClose={close} title="เพิ่มสินค้า">
      <Box>
        <SKUUpsertFormProvider
          initValue={{ name: '', price: 0, type: '' }}
          onSubmit={handleSubmit}
        >
          <SKUUpertFormFields />
          <ModalFormSubmissionButton
            onClose={close}
            cancelText="อิอิ"
            confirmText="confirmeiei"
          />
        </SKUUpsertFormProvider>
      </Box>
    </Modal>
  );
};

export default ModalUpsertSKU;
