import { Paper } from '@mantine/core';
import { TestFormProvider, TestFormInput } from './TestFormContext';
import TestFormFields from './TestFormFields';

const Test = () => {
  const handleSubmit = (data: TestFormInput) => {
    console.log(data);
  };

  return (
    <Paper shadow="xs" radius="md" p="md" mt="md">
      <div>this is a test page na</div>
      <TestFormProvider
        onSubmit={handleSubmit}
        initValue={{ name: 'sssss', price: 200 }}
      >
        <TestFormFields />
        <button type="submit">submit</button>
      </TestFormProvider>
    </Paper>
  );
};

export default Test;
